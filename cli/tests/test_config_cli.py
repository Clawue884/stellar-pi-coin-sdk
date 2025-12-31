import pytest
from click.testing import CliRunner
from cli.config_cli import show_config

def test_show_config():
    runner = CliRunner()
    result = runner.invoke(show_config)
    assert "pi_value_usd" in result.output
