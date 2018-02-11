defmodule SaddlePoints do
  @doc """
  Parses a string representation of a matrix
  to a list of rows
  """
  @spec rows(String.t()) :: [[integer]]
  def rows(str) do
    str
    |> String.split("\n")
    |> Enum.map(&parse_row/1)
  end

  @doc """
  Parses a string representation of a matrix
  to a list of columns
  """
  @spec columns(String.t()) :: [[integer]]
  def columns(str) do
    str
    |> rows
    |> transpose
  end

  @doc """
  Calculates all the saddle points from a string
  representation of a matrix
  """
  @spec saddle_points(String.t()) :: [{integer, integer}]
  def saddle_points(str) do
    rows_with_point =
      str
      |> rows
      |> with_point

    min_column_points =
      rows_with_point
      |> transpose
      |> Enum.flat_map(&select_column_elements/1)
      |> Enum.map(& &1[:point])

    rows_with_point
    |> Enum.flat_map(&select_row_elements/1)
    |> Enum.filter(&Enum.member?(min_column_points, &1[:point]))
    |> Enum.map(& &1[:point])
  end

  defp parse_row(row) do
    row
    |> String.split()
    |> Enum.map(&String.to_integer/1)
  end

  defp transpose(rows) do
    rows
    |> List.zip()
    |> Enum.map(&Tuple.to_list/1)
  end

  defp with_point(rows) do
    rows
    |> Enum.with_index()
    |> Enum.map(fn {list, row} ->
      list
      |> Enum.with_index()
      |> Enum.map(fn {n, col} -> %{amount: n, point: {row, col}} end)
    end)
  end

  defp select_column_elements(elements) do
    select_elements(elements, &Enum.min/2)
  end

  defp select_row_elements(elements) do
    select_elements(elements, &Enum.max/2)
  end

  defp select_elements(elements, selector) do
    selected_amount =
      elements
      |> Enum.map(& &1[:amount])
      |> selector.(fn -> raise(Enum.EmptyError) end)

    elements
    |> Enum.filter(&(&1[:amount] == selected_amount))
  end
end
