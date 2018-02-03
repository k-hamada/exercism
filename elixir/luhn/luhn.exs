defmodule Luhn do
  import Integer, only: [is_odd: 1]

  @doc """
  Checks if the given number is valid via the luhn formula
  """
  @spec valid?(String.t()) :: boolean
  def valid?(number) do
    with true <- has_many_digits?(number),
         true <- only_digit_or_space?(number),
         do: number |> parse |> check == 0
  end

  defp has_many_digits?(number) do
    Regex.scan(~r/\d/, number) |> Enum.count() > 1
  end

  defp only_digit_or_space?(number) do
    not String.match?(number, ~r/[^\d\s]/)
  end

  defp parse(number) do
    number
    |> String.graphemes()
    |> Enum.reject(&(&1 == " "))
    |> Enum.map(&String.to_integer/1)
  end

  defp check(numbers) do
    numbers
    |> Enum.reverse()
    |> Enum.with_index(1)
    |> Enum.flat_map(&calc/1)
    |> Enum.sum()
    |> rem(10)
  end

  defp calc({n, i}) when is_odd(i), do: [n]
  defp calc({n, _}) when n * 2 >= 10, do: [1, rem(n * 2, 10)]
  defp calc({n, _}), do: [n * 2]
end
