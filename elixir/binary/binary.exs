defmodule Binary do
  @doc """
  Convert a string containing a binary number to an integer.

  On errors returns 0.
  """
  @spec to_decimal(String.t()) :: non_neg_integer
  def to_decimal(string) do
    binary_to_decimal(string, 0)
  end

  defp binary_to_decimal("", acc), do: acc
  defp binary_to_decimal("0" <> tail, acc), do: binary_to_decimal(tail, acc * 2)
  defp binary_to_decimal("1" <> tail, acc), do: binary_to_decimal(tail, acc * 2 + 1)
  defp binary_to_decimal(_, _), do: 0
end
