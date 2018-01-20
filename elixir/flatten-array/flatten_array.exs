defmodule FlattenArray do
  @doc """
    Accept a list and return the list flattened without nil values.

    ## Examples

      iex> FlattenArray.flatten([1, [2], 3, nil])
      [1,2,3]

      iex> FlattenArray.flatten([nil, nil])
      []

  """

  @spec flatten(list) :: list
  def flatten(list) do
    flatten_impl(list)
  end

  defp flatten_impl([h | t]) do
    flatten_impl(h) ++ flatten_impl(t)
  end

  defp flatten_impl([]), do: []
  defp flatten_impl(nil), do: []
  defp flatten_impl(v), do: [v]
end
