.PHONY: install
install:
	@npx vsce package --allow-missing-repository
	@code --install-extension vscode-prosedown-0.0.1.vsix

.PHONY: uninstall
uninstall:
	@rm -f ~/.vscode/extensions/vscode-prosedown
