defmodule Tournament do
  @doc """
  Given `input` lines representing two teams and whether the first of them won,
  lost, or reached a draw, separated by semicolons, calculate the statistics
  for each team's number of games played, won, drawn, lost, and total points
  for the season, and return a nicely-formatted string table.

  A win earns a team 3 points, a draw earns 1 point, and a loss earns nothing.

  Order the outcome by most total points for the season, and settle ties by
  listing the teams in alphabetical order.
  """
  @spec tally(input :: list(String.t())) :: String.t()
  def tally(input) do
    input
    |> Enum.map(&parse/1)
    |> Enum.reduce(%{}, &register/2) |> Map.values
    |> Enum.sort(&order/2)
    |> output
  end

  defp parse(input) do
    Regex.named_captures(~r/(?<teama>[\p{L}|\s]+);(?<teamb>[\p{L}|\s]+);(?<result>(win|loss|draw))$/, input)
  end

  defp register(nil, acc), do: acc
  defp register(%{"teama" => teama, "teamb" => teamb, "result" => result}, acc) do
    acc
    |> Map.put_new(teama, new_record(teama))
    |> Map.get_and_update(teama, fn r -> {r, update_team_result(r, result)} end) |> elem(1)
    |> Map.put_new(teamb, new_record(teamb))
    |> Map.get_and_update(teamb, fn r -> {r, update_opponent_result(r, result)} end) |> elem(1)
  end

  defp new_record(name) do
    %{name: name, played: 0, won: 0, drawn: 0, lost: 0, points: 0}
  end

  defp update_team_result(record, "win"),  do: record |> update_point(3) |> update_count(:won)
  defp update_team_result(record, "draw"), do: record |> update_point(1) |> update_count(:drawn)
  defp update_team_result(record, "loss"), do: record |> update_point(0) |> update_count(:lost)
  defp update_opponent_result(record, "win"),  do: update_team_result(record, "loss")
  defp update_opponent_result(record, "draw"), do: update_team_result(record, "draw")
  defp update_opponent_result(record, "loss"), do: update_team_result(record, "win")

  defp update_point(record, amount) do
    record
    |> update_in([:points], &(&1 + amount))
  end

  defp update_count(record, result) do
    record
    |> update_in([:played], &(&1 + 1))
    |> update_in([result], &(&1 + 1))
  end

  defp order(%{points: points_a}, %{points: points_b}) when points_a != points_b do
    points_a > points_b
  end

  defp order(%{name: name_a}, %{name: name_b}) do
    name_a < name_b
  end

  defp output(records) do
    header = %{name: "Team", played: "MP", won: "W", drawn: "D", lost: "L", points: "P"}
    [header | records] |> Enum.map(&output_record/1) |> Enum.join("\n")
  end

  defp output_record(%{name: team, played: mp, won: w, drawn: d, lost: l, points: p}) do
    [
      String.pad_trailing(team, 30),
      String.pad_leading(to_string(mp), 2),
      String.pad_leading(to_string(w), 2),
      String.pad_leading(to_string(d), 2),
      String.pad_leading(to_string(l), 2),
      String.pad_leading(to_string(p), 2)
    ]
    |> Enum.join(" | ")
  end
end
