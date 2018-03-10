defmodule Sieve do
  @doc """
  Generates a list of primes up to a given limit.
  """
  @spec primes_to(non_neg_integer) :: [non_neg_integer]
  def primes_to(limit) do
    do_primes_to()
    |> Enum.take_while(&(&1 < limit))
  end

  defp do_primes_to do
    Stream.concat([2], Stream.iterate(3, &(&1 + 2)))
    |> Stream.unfold(fn acc ->
      p = acc |> Enum.at(0)
      a = acc |> Stream.filter(&(rem(&1, p) != 0))
      {p, a}
    end)
  end
end
