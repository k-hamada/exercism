defmodule SumOfMultiples do
  @doc """
  Adds up all numbers from 1 to a given end number that are multiples of the factors provided.
  """
  @spec to(non_neg_integer, [non_neg_integer]) :: non_neg_integer
  def to(limit, factors) do
    sequence = Stream.iterate(1, &(&1 + 1))
    factors
    |> Enum.map(fn(factor) -> Stream.map(sequence, &(factor * &1)) end)
    |> Enum.map(fn(stream) -> Stream.take_while(stream, &(limit > &1)) end)
    |> Stream.concat
    |> Stream.uniq
    |> Enum.sum
  end
end
