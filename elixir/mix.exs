defmodule ApiTemplate.MixProject do
  use Mix.Project

  def project do
    [
      app: :api_template,
      version: "0.1.0",
      elixir: "~> 1.18",
      deps: [{:bandit, "~> 1.0"}]
    ]
  end

  def application do
    [
      mod: {ApiTemplate.Application, []}
    ]
  end
end
