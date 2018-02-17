defmodule PrimeFactors do
  @doc """
  Compute the prime factors for 'number'.

  The prime factors are prime numbers that when multiplied give the desired
  number.

  The prime factors of 'number' will be ordered lowest to highest.
  """
  @spec factors_for(pos_integer) :: [pos_integer]
  def factors_for(number) do
    prime_factorization(number, 2, [])
    |> Enum.sort()
  end

  defp prime_factorization(number, i, primes) do
    cond do
      number < i ->
        primes

      Integer.mod(number, i) == 0 ->
        prime_factorization(div(number, i), i, [i | primes])

      true ->
        prime_factorization(number, i + 1, primes)
    end
  end
end
