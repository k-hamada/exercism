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

  defp aliquot_sum(number) do
    number
    |> divisor
    |> Enum.sum()
  end

  defp divisor(number) do
    1..trunc(:math.sqrt(number))
    |> Enum.filter(fn divider -> rem(number, divider) == 0 end)
    |> Enum.flat_map(fn factor -> [factor, div(number, factor)] end)
    |> Enum.uniq()
    |> Enum.reject(&(&1 == number))
  end

  defp placement(aliquot_sum, number) when aliquot_sum == number, do: :perfect
  defp placement(aliquot_sum, number) when aliquot_sum > number, do: :abundant
  defp placement(aliquot_sum, number) when aliquot_sum < number, do: :deficient
end
