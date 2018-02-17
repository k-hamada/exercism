defmodule BinarySearch do
  @doc """
    Searches for a key in the tuple using the binary search algorithm.
    It returns :not_found if the key is not in the tuple.
    Otherwise returns {:ok, index}.

    ## Examples

      iex> BinarySearch.search({}, 2)
      :not_found

      iex> BinarySearch.search({1, 3, 5}, 2)
      :not_found

      iex> BinarySearch.search({1, 3, 5}, 5)
      {:ok, 2}

  """

  @spec search(tuple, integer) :: {:ok, integer} | :not_found
  def search(numbers, key) do
    bsearch_index(numbers, key, 0, tuple_size(numbers) - 1)
  end

  defp bsearch_index({}, _, _, _) do
    :not_found
  end

  defp bsearch_index(_, _, min, max) when min > max do
    :not_found
  end

  defp bsearch_index(numbers, key, min, max) do
    c = (min + (max - min) / 2) |> trunc
    el = elem(numbers, c)

    cond do
      el == key -> {:ok, c}
      el > key -> bsearch_index(numbers, key, min, c - 1)
      el < key -> bsearch_index(numbers, key, c + 1, max)
    end
  end
end
