defmodule Hexadecimal do
  @doc """
    Accept a string representing a hexadecimal value and returns the
    corresponding decimal value.
    It returns the integer 0 if the hexadecimal is invalid.
    Otherwise returns an integer representing the decimal value.

    ## Examples

      iex> Hexadecimal.to_decimal("invalid")
      0

      iex> Hexadecimal.to_decimal("af")
      175

  """
  @base 16
  @digits ~w(0 1 2 3 4 5 6 7 8 9 A B C D E F)

  @spec to_decimal(binary) :: integer
  def to_decimal(hex) do
    hex
    |> String.upcase()
    |> String.graphemes()
    |> Enum.reverse()
    |> Enum.with_index()
    |> Enum.reduce_while(0, &to_decimal_and_sum/2)
  end

  defp to_decimal_and_sum({hex, index}, acc) do
    with n when is_number(n) <- @digits |> Enum.find_index(&(&1 == hex)),
         b when is_number(b) <- :math.pow(@base, index) do
      {:cont, n * b + acc}
    else
      _ -> {:halt, 0}
    end
  end
end
