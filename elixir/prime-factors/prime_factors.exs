defmodule PrimeFactors do
  @doc """
  Compute the prime factors for 'number'.

  The prime factors are prime numbers that when multiplied give the desired
  number.

  The prime factors of 'number' will be ordered lowest to highest.
  """
  @spec factors_for(pos_integer) :: [pos_integer]
  def factors_for(number) do
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
