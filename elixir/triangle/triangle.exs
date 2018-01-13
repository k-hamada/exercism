defmodule Triangle do
  @type kind :: :equilateral | :isosceles | :scalene

  @doc """
  Return the kind of triangle of a triangle with 'a', 'b' and 'c' as lengths.
  """
  @spec kind(number, number, number) :: { :ok, kind } | { :error, String.t }
  def kind(a, b, c) do
    triangle(a, b, c)
  end

  defp triangle(x, y, z) when x <= 0 or y <= 0 or z <= 0,
    do: {:error, "all side lengths must be positive"}
  defp triangle(x, y, z) when x + y <= z or y + z <= x or z + x <= y,
    do: {:error, "side lengths violate triangle inequality"}

  defp triangle(x, x, x), do: {:ok, :equilateral}
  defp triangle(x, x, _), do: {:ok, :isosceles}
  defp triangle(x, _, x), do: {:ok, :isosceles}
  defp triangle(_, x, x), do: {:ok, :isosceles}
  defp triangle(_, _, _), do: {:ok, :scalene}
end
