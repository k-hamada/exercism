defmodule Prime do

  @doc """
  Generates the nth prime.
  """
  @spec nth(non_neg_integer) :: non_neg_integer
  def nth(count) when 0 >= count, do: raise :error
  def nth(count) do
   Stream.iterate(2, &(&1 + 1))
   |> Stream.filter(&is_prime?/1)
   |> Enum.at(count - 1)
  end

  defp is_prime?(2), do: true
  defp is_prime?(n) when rem(n, 2) == 0, do: false
  defp is_prime?(n) do
    Stream.iterate(3, &(&1 + 1))
    |> Stream.take_while(&(&1 <= trunc(:math.sqrt(n))))
    |> Stream.map(&(rem(n, &1) != 0))
    |> Enum.all?
  end
end
