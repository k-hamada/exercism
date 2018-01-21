defmodule Grains do
  use Bitwise, only_operators: true

  @doc """
  Calculate two to the power of the input minus one.
  """
  @spec square(pos_integer) :: pos_integer
  def square(number) do
    square_impl(number)
  end

  defp square_impl(n) when 0 < n and n < 65 do
    {:ok, 1 <<< (n - 1)}
  end

  defp square_impl(_) do
    {:error, "The requested square must be between 1 and 64 (inclusive)"}
  end

  @doc """
  Adds square of each number from 1 to 64.
  """
  @spec total :: pos_integer
  def total do
    sum =
      1..64
      |> Enum.map(&square/1)
      |> Keyword.get_values(:ok)
      |> Enum.sum()

    {:ok, sum}
  end
end
