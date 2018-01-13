defmodule Triangle do
  @type kind :: :equilateral | :isosceles | :scalene

  @doc """
  Return the kind of triangle of a triangle with 'a', 'b' and 'c' as lengths.
  """
  @spec kind(number, number, number) :: { :ok, kind } | { :error, String.t }
  def kind(a, b, c) do
    triangle(a, b, c)
  end

  defp triangle(x, y, z) when x <= 0 or y <= 0 or z <= 0, do: {:error, "all side lengths must be positive"}

  defp triangle(x, x, x), do: equilateral(x, x, x)
  defp triangle(x, x, y), do: isosceles(x, x, y)
  defp triangle(x, y, x), do: isosceles(x, x, y)
  defp triangle(y, x, x), do: isosceles(x, x, y)
  defp triangle(x, y, z), do: scalene(x, y, z)

  defp equilateral(_, _, _), do: {:ok, :equilateral}

  defp isosceles(x, y, z) when x + y <= z, do: {:error, "side lengths violate triangle inequality"}
  defp isosceles(_, _, _), do: {:ok, :isosceles}

  defp scalene(x, y, z) when x + y <= z, do: {:error, "side lengths violate triangle inequality"}
  defp scalene(y, z, x) when x + y <= z, do: {:error, "side lengths violate triangle inequality"}
  defp scalene(z, x, y) when x + y <= z, do: {:error, "side lengths violate triangle inequality"}
  defp scalene(_, _, _), do: {:ok, :scalene}
end
