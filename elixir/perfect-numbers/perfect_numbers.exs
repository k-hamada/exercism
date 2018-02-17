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

  defp aliquot_sum(number) do
    number
    |> divisor
    |> Enum.sum()
  end

  defp divisor(1), do: []

  defp divisor(number) do
    case factors = factors_for(number) do
      [_] ->
        [1]

      _ ->
        factors
        |> combination_all
        |> Enum.map(&prod/1)
        |> Enum.uniq()
        |> Enum.concat([1])
    end
  end

  defp combination_all(xs) do
    1..(Enum.count(xs) - 1)
    |> Enum.flat_map(&combination(&1, xs))
  end

  defp combination(0, _), do: [[]]
  defp combination(_, []), do: []

  defp combination(n, [x | xs]),
    do: for(y <- combination(n - 1, xs), do: [x | y]) ++ combination(n, xs)

  defp prod(xs), do: xs |> Enum.reduce(&*/2)

  defp factors_for(number) do
    do_factors_for({2, number})
  end

  defp do_factors_for({d, n}) do
    Stream.unfold({d, n}, fn
      nil ->
        nil

      {d, n} ->
        Stream.iterate(d, &(&1 + 1))
        |> Stream.take_while(&(:math.pow(&1, 2) <= n))
        |> Stream.concat(for i <- [n], i > 1, do: i)
        |> Stream.filter(&(Integer.mod(n, &1) == 0))
        |> Stream.map(&{&1, {&1, div(n, &1)}})
        |> Enum.take(1)
        |> list_to_maybe
    end)
    |> Enum.to_list()
  end

  defp list_to_maybe([]), do: nil
  defp list_to_maybe(list), do: list |> List.first()
end
