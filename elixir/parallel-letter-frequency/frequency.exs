defmodule Frequency do
  @doc """
  Count letter frequency in parallel.

  Returns a map of characters to frequencies.

  The number of worker processes to use can be set with 'workers'.
  """
  @spec frequency([String.t()], pos_integer) :: map
  def frequency(texts, workers) do
    texts
    |> Task.async_stream(Frequency, :count, [], max_concurrency: workers)
    |> Enum.to_list()
    |> Enum.reduce(%{}, fn {:ok, map}, acc -> Map.merge(acc, map, fn _, v1, v2 -> v1 + v2 end) end)
  end

  def count(text) do
    text
    |> String.downcase()
    |> (&Regex.scan(~r/\p{L}/u, &1)).()
    |> Enum.reduce(%{}, fn [letter], acc -> Map.update(acc, letter, 1, &(&1 + 1)) end)
  end
end
