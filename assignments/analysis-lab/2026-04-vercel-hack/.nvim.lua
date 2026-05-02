-- Neovim config stub for Vercel Hack project
-- Requires rust-tools, tauri-apps/tauri-vscode equivalent plugins
vim.api.nvim_set_keymap('n', '<leader>tb', ':!cargo tauri build<CR>', { noremap = true, silent = true })
vim.api.nvim_set_keymap('n', '<leader>td', ':!cargo tauri dev<CR>', { noremap = true, silent = true })
vim.api.nvim_set_keymap('n', '<leader>tt', ':!cargo nextest run<CR>', { noremap = true, silent = true })
vim.api.nvim_set_keymap('n', '<leader>tl', ':!cargo clippy<CR>', { noremap = true, silent = true })
