defmodule AllYourBase do
  @doc """
  Given a number in base a, represented as a sequence of digits, converts it to base b,
  or returns nil if either of the bases are less than 2
  """
  @spec convert(list, integer, integer) :: list
  def convert(digits, base_a, base_b) do
    cond do
      is_fail?(digits, base_a, base_b) ->
        nil
      true ->
        to_base_n(to_base_ten(digits, base_a), base_b, [])
    end
  end

  defp is_fail?(digits, base_a, base_b) do
    digits == [] ||
    base_a <= 1 ||
    base_b <= 1 ||
    Enum.any?(digits, &(&1 < 0 || &1 >= base_a))
  end

  defp to_base_ten(digits, base) do
    digits
    |> Enum.reverse
    |> Enum.with_index
    |> Enum.map(fn {x, y} -> round(:math.pow(base, y) * x) end)
    |> Enum.sum
  end

  defp to_base_n(0, _, []), do: [0]
  defp to_base_n(0, _, acc), do: acc
  defp to_base_n(number, base, acc) do
    to_base_n(div(number, base), base, [rem(number, base) | acc])
  end
end
