defmodule Gigasecond do
  @doc """
  Calculate a date one billion seconds after an input date.
  """
  @epoch ~N[1970-01-01 00:00:00]
  @spec from({{pos_integer, pos_integer, pos_integer}, {pos_integer, pos_integer, pos_integer}}) :: :calendar.datetime()
  def from(erl) do
    with {:ok, naive_date_time} <- NaiveDateTime.from_erl(erl),
         do: naive_date_time |> add_gigasecond
  end

  defp add_gigasecond(naive_date_time) do
    naive_date_time
    |> NaiveDateTime.diff(@epoch)
    |> (&NaiveDateTime.add(@epoch, &1 + 1_000_000_000)).()
    |> NaiveDateTime.to_erl()
  end
end
