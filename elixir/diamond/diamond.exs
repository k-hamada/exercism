defmodule Diamond do
  @doc """
  Given a letter, it prints a diamond starting with 'A',
  with the supplied letter at the widest point.
  """
  @spec build_shape(char) :: String.t()
  def build_shape(letter) do
    letters = ?A..letter
    length = Enum.count(letters)

    spaces =
      1..length
      |> Enum.map(&(length - &1))
      |> Enum.map(&[&1, length - &1 - 1])
      |> Enum.map(&Enum.map(&1, fn n -> List.duplicate(" ", n) end))

    left =
      letters
      |> Enum.zip(spaces)
      |> Enum.map(fn {l, [s1, s2]} -> Enum.concat([s1, [[l]], s2]) end)

    right =
      left
      |> Enum.map(&Enum.reverse/1)
      |> Enum.map(&Enum.drop(&1, 1))

    top =
      Enum.zip(left, right)
      |> Enum.map(&Tuple.to_list/1)
      |> Enum.map(&Enum.join(&1))

    bottom =
      top
      |> Enum.reverse()
      |> Enum.drop(1)
      |> List.insert_at(-1, "")

    [top, bottom]
    |> Enum.map(&Enum.join(&1, "\n"))
    |> Enum.join("\n")
  end
end
