defmodule Gigasecond do
  @doc """
  Calculate a date one billion seconds after an input date.
  """
  @spec from({{pos_integer, pos_integer, pos_integer}, {pos_integer, pos_integer, pos_integer}}) :: :calendar.datetime()
  def from(erl) do
    with {:ok, naive_date_time} <- NaiveDateTime.from_erl(erl),
         do: naive_date_time |> NaiveDateTime.add(1_000_000_000) |> NaiveDateTime.to_erl()
  end
end
