import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';

import { openSettingsDialog } from './settingsEvents';

import { MissingApiKeyHint } from './MissingApiKeyHint';

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

vi.mock('./settingsEvents', () => ({
  openSettingsDialog: vi.fn(),
}));

describe('MissingApiKeyHint', () => {
  it('opens supplier configuration instead of built-in API configuration', async () => {
    const user = userEvent.setup();

    render(<MissingApiKeyHint />);

    await user.click(screen.getByRole('button', { name: 'settings.openSuppliersSettings' }));

    expect(openSettingsDialog).toHaveBeenCalledWith({ category: 'suppliers' });
  });
});
