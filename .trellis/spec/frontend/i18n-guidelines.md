# i18n 规范

## 入口与语言文件

- i18n 入口：`src/i18n/index.ts`
- 中文语言文件：`src/i18n/locales/zh.json`
- 英文语言文件：`src/i18n/locales/en.json`

组件中统一使用 `useTranslation()` 和 `t('key.path')`，避免硬编码中英文文案。

## Key 命名

- 使用模块化层级命名，例如 `project.title`、`node.menu.uploadImage`、`common.save`。
- 避免把中文句子直接作为 key；key 必须稳定、可复用、可检索。
- 通用文案优先放 `common.*`，页面专属文案放对应模块前缀。

## 新增文案流程

1. 先在 `zh.json` 增加新 key。
2. 同步在 `en.json` 增加相同 key。
3. 代码里只引用 key，不写 fallback 字面量。

## 动态值与复数

- 动态值用插值：`t('xxx', { count, name })`。
- 数量相关场景使用 i18next 复数规则，不手写字符串拼接。
- 数字、时间等先格式化，再传给 `t`。

## 最低验证

- 切换中英文后，不出现未翻译 key 泄露，例如直接显示 `project.title`。
- 新增 key 在中英语言包均存在。
- 关键按钮、提示、错误文案在两种语言下都可读、不截断。
