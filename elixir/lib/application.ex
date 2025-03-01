defmodule ApiTemplate.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      {Bandit, plug: ApiTemplate.Router, port: 3000}
    ]

    Supervisor.start_link(children, strategy: :one_for_one, name: ApiTemplate.Supervisor)
  end
end
