defmodule PascalsTriangle do
  @doc """
  Calculates the rows of a pascal triangle
  with the given height
  """
  @spec rows(integer) :: [[integer]]
  def rows(num) do
    1..num
    |> Enum.reduce([], &get_row/2)
    |> Enum.reverse()
  end

  defp get_row(1, _) do
    [[1]]
  end

  defp get_row(_, [head | tail]) do
    [next_row(head) | [head | tail]]
  end

  defp next_row(list) do
    list
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.map(&Enum.sum/1)
    |> List.insert_at(0, 1)
    |> List.insert_at(-1, 1)
  end
end
