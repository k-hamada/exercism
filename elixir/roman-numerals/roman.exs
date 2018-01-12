defmodule Roman do
  @doc """
  Convert the number to a roman number.
  """
  @spec numerals(pos_integer) :: String.t
  def numerals(number) do
    romanize(rem(div(number, 1000), 10), %{:one => "M", :five => "",  :ten => "" }) <>
    romanize(rem(div(number, 100), 10),  %{:one => "C", :five => "D", :ten => "M"}) <>
    romanize(rem(div(number, 10), 10),   %{:one => "X", :five => "L", :ten => "C"}) <>
    romanize(rem(div(number, 1), 10),    %{:one => "I", :five => "V", :ten => "X"})
  end

  defp romanize(n, table) when n in 1..3, do: String.duplicate(table[:one], n)
  defp romanize(n, table) when n in 4..4, do: table[:one] <> table[:five]
  defp romanize(n, table) when n in 5..5, do: table[:five]
  defp romanize(n, table) when n in 6..8, do: table[:five] <> String.duplicate(table[:one], n - 5)
  defp romanize(n, table) when n in 9..9, do: table[:one] <> table[:ten]
  defp romanize(_, _), do: ""
end
