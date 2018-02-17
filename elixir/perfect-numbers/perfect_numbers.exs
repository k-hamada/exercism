defmodule PerfectNumbers do
  @doc """
  Determine the aliquot sum of the given `number`, by summing all the factors
  of `number`, aside from `number` itself.

  Based on this sum, classify the number as:

  :perfect if the aliquot sum is equal to `number`
  :abundant if the aliquot sum is greater than `number`
  :deficient if the aliquot sum is less than `number`
  """
  @spec classify(number :: integer) :: {:ok, atom} | {:error, String.t()}
  def classify(number) do
    with number when number > 0 <- number,
         class <- number |> aliquot_sum |> placement(number) do
      {:ok, class}
    else
      _ -> {:error, "Classification is only possible for natural numbers."}
    end
  end

  defp placement(aliquot_sum, number) do
    cond do
      aliquot_sum == number -> :perfect
      aliquot_sum > number -> :abundant
      aliquot_sum < number -> :deficient
    end
  end

  defp aliquot_sum(number) when number <= 0, do: 0

  defp aliquot_sum(number) do
    number
    |> divisor
    |> Enum.sum()
  end

  defp divisor(1), do: []
  defp divisor(2), do: [1]

  defp divisor(number) do
    2..(div(number, 2) + 1)
    |> Enum.filter(&(rem(number, &1) == 0))
    |> Enum.concat([1])
  end
end
