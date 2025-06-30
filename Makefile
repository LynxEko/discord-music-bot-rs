.PHONY: config-dev config-release


config-dev:
	@cp secret/config_dev.toml config.toml
	@echo "copied 'secret/config_dev.toml' as 'config.toml'"

config-prod:
	@cp secret/config_prod.toml config.toml
	@echo "copied 'secret/config_prod.toml' as 'config.toml'"

