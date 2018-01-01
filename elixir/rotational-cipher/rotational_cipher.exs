defmodule RotationalCipher do
  @doc """
  Given a plaintext and amount to shift by, return a rotated string.

  Example:
  iex> RotationalCipher.rotate("Attack at dawn", 13)
  "Nggnpx ng qnja"
  """
  @spec rotate(text :: String.t(), shift :: integer) :: String.t()
  def rotate(text, shift) do
    text
    |> to_charlist
    |> Enum.map(&(char_rotate(&1, shift)))
    |> List.to_string
  end

  def char_rotate(char, shift) when char in ?A..?Z do
    char_rotate(char, shift, ?A)
  end

  def char_rotate(char, shift) when char in ?a..?z do
    char_rotate(char, shift, ?a)
  end

  def char_rotate(char, _) do
    char
  end

  def char_rotate(eacute, shift, basecodepoint) do
    <<rem(eacute + shift - basecodepoint, 26) + basecodepoint>>
  end
end
