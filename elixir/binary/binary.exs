defmodule Binary do
  @doc """
  Convert a string containing a binary number to an integer.

  On errors returns 0.
  """
  @spec to_decimal(String.t()) :: non_neg_integer
  def to_decimal(string) do
    string
    |> parse
    |> Enum.reverse()
    |> Enum.with_index()
    |> Enum.map(&calc_decimal/1)
    |> Enum.sum()
  end

  defp parse(binary) do
    if String.match?(binary, ~r/^[0|1]+$/), do: binary |> String.graphemes(), else: ["0"]
  end

  defp calc_decimal({"0", _}), do: 0
  defp calc_decimal({"1", i}), do: :math.pow(2, i)
end
