defmodule Roman do
  @doc """
  Convert the number to a roman number.
  """
  @spec numerals(pos_integer) :: String.t
  def numerals(number) do
    romanize(div(number, 1000),         %{:_1 => "M", :_5 => "",  :_a => "" }) <>
    romanize(div(number, 100),          %{:_1 => "C", :_5 => "D", :_a => "M"}) <>
    romanize(div(rem(number, 100), 10), %{:_1 => "X", :_5 => "L", :_a => "C"}) <>
    romanize(rem(number, 10),           %{:_1 => "I", :_5 => "V", :_a => "X"})
  end

  defp romanize(n, table) when n in 1..3, do: String.duplicate(table[:_1], n)
  defp romanize(n, table) when n in 4..4, do: table[:_1] <> table[:_5]
  defp romanize(n, table) when n in 5..5, do: table[:_5]
  defp romanize(n, table) when n in 6..8, do: table[:_5] <> String.duplicate(table[:_1], n - 5)
  defp romanize(n, table) when n in 9..9, do: table[:_1] <> table[:_a]
  defp romanize(_, _), do: ""
end
