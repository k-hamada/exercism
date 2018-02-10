defmodule PascalsTriangle do
  @doc """
  Calculates the rows of a pascal triangle
  with the given height
  """
  @spec rows(integer) :: [[integer]]
  def rows(num) do
    [1]
    |> Stream.iterate(&next_row/1)
    |> Enum.take(num)
  end

  defp next_row(list) do
    list
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.map(&Enum.sum/1)
    |> List.insert_at(0, 1)
    |> List.insert_at(-1, 1)
  end
end
