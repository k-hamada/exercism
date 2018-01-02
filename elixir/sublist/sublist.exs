defmodule Sublist do
  @doc """
  Returns whether the first list is a sublist or a superlist of the second list
  and if not whether it is equal or unequal to the second list.
  """
  def compare(a, b) do
    cond do
      is_equal(a, b) -> :equal
      is_sublist(a, b) -> :sublist
      is_superlist(a, b) -> :superlist
      true -> :unequal
    end
  end

  defp is_equal(a, b) do
    a === b
  end

  defp is_sublist([], _), do: true
  defp is_sublist(_, []), do: false
  defp is_sublist(a, b) do
    index = Enum.find_index(b, fn(e) -> e === hd(a) end)
    size = Enum.count(a)
    is_sublist(a, b, index, size)
  end

  defp is_sublist(_, _, nil, _), do: false
  defp is_sublist(a, b, index, size) do
    if is_equal(a, b |> Enum.drop(index) |> Enum.take(size)) do
      true
    else
      is_sublist(a, tl(b))
    end
  end

  defp is_superlist(a, b) do
    is_sublist(b, a)
  end
end
