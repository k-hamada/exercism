defmodule Luhn do
  @doc """
  Checks if the given number is valid via the luhn formula
  """
  @spec valid?(String.t()) :: boolean
  def valid?(number) do
    number |> parse |> check
  end

  defp parse(number) do
    if String.match?(number, ~r/[^\d|\s]/) do
      []
    else
      number
      |> String.graphemes()
      |> Enum.reject(&(&1 == " "))
      |> Enum.map(&String.to_integer/1)
      |> Enum.reverse()
    end
  end

  defp check(numbers) when length(numbers) <= 1 do
    false
  end

  defp check(numbers) do
    numbers
    |> Enum.with_index(1)
    |> Enum.flat_map(&calc/1)
    |> Enum.sum()
    |> rem(10) == 0
  end

  defp calc({n, i}) when rem(i, 2) != 0, do: [n]

  defp calc({n, _}) do
    res = n * 2

    if res >= 10 do
      [1, rem(res, 10)]
    else
      [res]
    end
  end
end
