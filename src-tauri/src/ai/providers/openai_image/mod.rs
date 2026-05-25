use base64::{engine::general_purpose, Engine as _};
use reqwest::{multipart, Client};
use serde_json::{json, Value};

use crate::ai::error::AIError;
use crate::ai::{GenerateRequest, RuntimeProviderConfig};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenAiEditImage {
    pub mime_type: String,
    pub bytes: Vec<u8>,
}

fn trim(value: Option<&str>) -> String {
    value.unwrap_or_default().trim().to_string()
}

fn validate_runtime(runtime: &RuntimeProviderConfig) -> Result<(String, String, String), AIError> {
    let base_url = trim(runtime.base_url.as_deref()).trim_end_matches('/').to_string();
    let api_key = trim(runtime.api_key.as_deref());
    let remote_model_id = trim(runtime.remote_model_id.as_deref());

    if base_url.is_empty() {
        return Err(AIError::InvalidRequest(
            "custom openai-image provider missing baseUrl".to_string(),
        ));
    }
    if api_key.is_empty() {
        return Err(AIError::InvalidRequest(
            "custom openai-image provider missing apiKey".to_string(),
        ));
    }
    if remote_model_id.is_empty() {
        return Err(AIError::InvalidRequest(
            "custom openai-image provider missing remoteModelId".to_string(),
        ));
    }

    Ok((base_url, api_key, remote_model_id))
}

fn build_final_prompt(request: &GenerateRequest) -> String {
    let prompt = request.prompt.trim();
    let aspect_ratio = request.aspect_ratio.trim();
    if aspect_ratio.is_empty() || aspect_ratio.eq_ignore_ascii_case("auto") {
        return prompt.to_string();
    }

    format!("{prompt}\nImage aspect ratio: {aspect_ratio}")
}

pub fn build_generation_body(
    request: &GenerateRequest,
    runtime: &RuntimeProviderConfig,
) -> Result<Value, AIError> {
    let (_, _, remote_model_id) = validate_runtime(runtime)?;

    Ok(json!({
        "model": remote_model_id,
        "prompt": build_final_prompt(request),
        "n": 1
    }))
}

fn parse_data_url_image(value: &str) -> Result<OpenAiEditImage, AIError> {
    let (metadata, payload) = value
        .split_once(',')
        .ok_or_else(|| {
            AIError::InvalidRequest(
                "custom openai-image reference images must be data URLs".to_string(),
            )
        })?;
    let metadata = metadata.trim();
    if !metadata.starts_with("data:") || !metadata.ends_with(";base64") {
        return Err(AIError::InvalidRequest(
            "custom openai-image reference images must be base64 data URLs".to_string(),
        ));
    }

    let mime_type = metadata
        .trim_start_matches("data:")
        .trim_end_matches(";base64")
        .trim();
    if mime_type.is_empty() {
        return Err(AIError::InvalidRequest(
            "custom openai-image reference image missing MIME type".to_string(),
        ));
    }

    let bytes = general_purpose::STANDARD
        .decode(payload.trim())
        .map_err(|error| {
            AIError::InvalidRequest(format!(
                "custom openai-image reference image base64 decode failed: {}",
                error
            ))
        })?;

    Ok(OpenAiEditImage {
        mime_type: mime_type.to_string(),
        bytes,
    })
}

pub fn collect_edit_images(request: &GenerateRequest) -> Result<Vec<OpenAiEditImage>, AIError> {
    let images = request
        .reference_images
        .as_ref()
        .map(|values| {
            values
                .iter()
                .map(|value| value.trim())
                .filter(|value| !value.is_empty())
                .map(parse_data_url_image)
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    if images.is_empty() {
        return Err(AIError::InvalidRequest(
            "custom openai-image edit requires reference images".to_string(),
        ));
    }

    Ok(images)
}

fn image_extension(mime_type: &str) -> &'static str {
    match mime_type {
        "image/jpeg" => "jpg",
        "image/webp" => "webp",
        "image/gif" => "gif",
        _ => "png",
    }
}

fn build_edit_form(
    request: &GenerateRequest,
    runtime: &RuntimeProviderConfig,
) -> Result<multipart::Form, AIError> {
    let (_, _, remote_model_id) = validate_runtime(runtime)?;
    let images = collect_edit_images(request)?;
    let mut form = multipart::Form::new()
        .text("model", remote_model_id)
        .text("prompt", build_final_prompt(request))
        .text("n", "1");

    for (index, image) in images.into_iter().enumerate() {
        let file_name = format!(
            "reference-{}.{}",
            index + 1,
            image_extension(image.mime_type.as_str())
        );
        let part = multipart::Part::bytes(image.bytes)
            .file_name(file_name)
            .mime_str(image.mime_type.as_str())?;
        form = form.part("image[]", part);
    }

    Ok(form)
}

pub fn parse_image_response(value: &Value) -> Result<String, AIError> {
    if let Some(payload) = value
        .pointer("/data/0/b64_json")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        return Ok(format!("data:image/png;base64,{}", payload));
    }

    if let Some(url) = value
        .pointer("/data/0/url")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        return Ok(url.to_string());
    }

    Err(AIError::Provider(
        "OpenAI Images response missing data[0].b64_json or data[0].url".to_string(),
    ))
}

pub async fn generate(
    request: &GenerateRequest,
    runtime: &RuntimeProviderConfig,
) -> Result<String, AIError> {
    let (base_url, api_key, _) = validate_runtime(runtime)?;
    let endpoint = format!("{}/images/generations", base_url);
    let body = build_generation_body(request, runtime)?;

    let response = Client::new()
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?
        .error_for_status()?;

    let body = response.json::<Value>().await?;
    parse_image_response(&body)
}

pub async fn edit(
    request: &GenerateRequest,
    runtime: &RuntimeProviderConfig,
) -> Result<String, AIError> {
    let (base_url, api_key, _) = validate_runtime(runtime)?;
    let endpoint = format!("{}/images/edits", base_url);
    let form = build_edit_form(request, runtime)?;

    let response = Client::new()
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()
        .await?
        .error_for_status()?;

    let body = response.json::<Value>().await?;
    parse_image_response(&body)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::ai::{GenerateRequest, RuntimeProviderConfig};

    use super::{
        build_generation_body, collect_edit_images, parse_image_response,
    };

    fn runtime() -> RuntimeProviderConfig {
        RuntimeProviderConfig {
            kind: "custom-provider".to_string(),
            provider_profile_id: Some("openai-images".to_string()),
            provider_display_name: Some("OpenAI Images".to_string()),
            protocol: Some("openai-image".to_string()),
            base_url: Some("https://api.openai.com/v1".to_string()),
            api_key: Some("sk-openai".to_string()),
            submit_base_url: None,
            wait_base_url: None,
            asset_base_url: None,
            default_output_format: None,
            remote_model_id: Some("gpt-image-1".to_string()),
        }
    }

    fn request() -> GenerateRequest {
        GenerateRequest {
            prompt: "turn into a cinematic poster".to_string(),
            model: "custom-provider:openai-images:gpt-image".to_string(),
            size: "1K".to_string(),
            aspect_ratio: "16:9".to_string(),
            action: None,
            reference_images: None,
            extra_params: None,
            provider_runtime: None,
        }
    }

    #[test]
    fn build_generation_body_maps_prompt_model_and_count() {
        let body = build_generation_body(&request(), &runtime()).expect("body");

        assert_eq!(body["model"], "gpt-image-1");
        assert!(body["prompt"]
            .as_str()
            .expect("prompt")
            .contains("turn into a cinematic poster"));
        assert_eq!(body["n"], 1);
    }

    #[test]
    fn collect_edit_images_maps_data_url_reference_images() {
        let mut request = request();
        request.reference_images = Some(vec![
            "data:image/png;base64,YWJj".to_string(),
        ]);

        let references = collect_edit_images(&request).expect("references");

        assert_eq!(references.len(), 1);
        assert_eq!(references[0].mime_type, "image/png");
        assert_eq!(references[0].bytes, b"abc");
    }

    #[test]
    fn collect_edit_images_rejects_missing_reference_images() {
        let error = collect_edit_images(&request()).expect_err("missing refs should fail");

        assert!(error.to_string().contains("reference images"));
    }

    #[test]
    fn parse_image_response_prefers_b64_json() {
        let result = parse_image_response(&json!({
            "data": [{ "b64_json": "abc123", "url": "https://example.com/result.png" }]
        }))
        .expect("image");

        assert_eq!(result, "data:image/png;base64,abc123");
    }

    #[test]
    fn parse_image_response_accepts_url() {
        let result = parse_image_response(&json!({
            "data": [{ "url": "https://example.com/result.png" }]
        }))
        .expect("image");

        assert_eq!(result, "https://example.com/result.png");
    }
}
