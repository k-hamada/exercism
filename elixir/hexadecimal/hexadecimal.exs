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
    case hex_to_decimal(hex) do
      {:ok, n} -> n
      {:error} -> 0
    end
  end

  defp hex_to_decimal(hex) do
    hex
    |> String.upcase()
    |> String.graphemes()
    |> Enum.reverse()
    |> Enum.with_index()
    |> Enum.map(&to_decimal_with_index/1)
    |> Enum.reduce(&sum/2)
  end

  defp sum(_, {:error}), do: {:error}
  defp sum({:error}, _), do: {:error}
  defp sum({:ok, a}, {:ok, b}), do: {:ok, a + b}

  defp to_decimal_with_index({hex, index}) do
    with n when is_number(n) <- to_digits(hex),
         b when is_number(b) <- :math.pow(@base, index) do
      {:ok, n * b}
    else
      _ -> {:error}
    end
  end

  defp to_digits(hex) do
    @digits
    |> Enum.find_index(&(&1 == hex))
  end
end
