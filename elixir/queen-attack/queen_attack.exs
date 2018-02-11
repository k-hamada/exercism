defmodule Queens do
  @type t :: %Queens{black: {integer, integer}, white: {integer, integer}}
  defstruct black: nil, white: nil

  @doc """
  Creates a new set of Queens
  """
  @spec new() :: Queens.t()
  @spec new({integer, integer}, {integer, integer}) :: Queens.t()
  def new(white \\ {0, 3}, black \\ {7, 3}) do
    new_queens(white, black)
  end

  defp new_queens(white, white) do
    raise ArgumentError
  end

  defp new_queens(white, black) do
    %Queens{white: white, black: black}
  end

  @doc """
  Gives a string reprentation of the board with
  white and black queen locations shown
  """
  @spec to_string(Queens.t()) :: String.t()
  def to_string(queens) do
    with board <- new_board(),
         {_, board} <- set_to_board(board, queens.white, "W"),
         {_, board} <- set_to_board(board, queens.black, "B"),
         do: board |> board_to_string
  end

  defp set_to_board(board, {row, col}, symbol) do
    Map.get_and_update(board, row, fn board_row ->
      {board_row, board_row |> List.update_at(col, fn _ -> symbol end)}
    end)
  end

  defp new_board() do
    Enum.reduce(0..7, %{}, fn i, acc -> Map.put(acc, i, List.duplicate("_", 8)) end)
  end

  defp board_to_string(board) do
    board
    |> Map.values()
    |> Enum.map(&Enum.join(&1, " "))
    |> Enum.join("\n")
  end

  @doc """
  Checks if the queens can attack each other
  """
  @spec can_attack?(Queens.t()) :: boolean
  def can_attack?(queens) do
    can_attack?(queens.white, queens.black)
  end

  defp can_attack?({row, _}, {row, _}), do: true
  defp can_attack?({_, col}, {_, col}), do: true

  defp can_attack?({w_row, w_col}, {b_row, b_col}) do
    abs(w_row - w_col) == abs(b_row - b_col)
  end
end
