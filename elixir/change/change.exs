defmodule Change do
  @doc """
    Determine the least number of coins to be given to the user such
    that the sum of the coins' value would equal the correct amount of change.
    It returns {:error, "cannot change"} if it is not possible to compute the
    right amount of coins. Otherwise returns the tuple {:ok, list_of_coins}

    ## Examples

      iex> Change.generate([5, 10, 15], 3)
      {:error, "cannot change"}

      iex> Change.generate([1, 5, 10], 18)
      {:ok, [1, 1, 1, 5, 10]}

  """

  @spec generate(list, integer) :: {:ok, list} | {:error, String.t()}
  def generate(coins, target) do
    calculate(%{0 => []}, coins, 0, target)
  end

  defp calculate(_, _, index, target) when index > target do
    {:error, "cannot change"}
  end

  defp calculate(payable, coins, index, target) do
    if Map.has_key?(payable, target) do
      {:ok, Enum.sort(payable[target])}
    else
      payable_plus_one =
        Enum.reduce(payable, %{}, fn {money, howto}, payable_acc ->
          Enum.reduce(coins, payable_acc, fn coin, coins_acc ->
            Map.put(coins_acc, money + coin, [coin | howto])
          end)
        end)

      calculate(payable_plus_one, coins, index + 1, target)
    end
  end
end
