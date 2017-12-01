defmodule RotationalCipher do
  @doc """
  Given a plaintext and amount to shift by, return a rotated string.

  Example:
  iex> RotationalCipher.rotate("Attack at dawn", 13)
  "Nggnpx ng qnja"
  """
  @spec rotate(text :: String.t(), shift :: integer) :: String.t()
  def rotate(text, shift) do
    String.codepoints(text)
    |> Enum.map(&(char_rotate(&1, shift)))
    |> List.to_string
  end

  def char_rotate(char, shift) do
    cond do
      char =~ ~r/[A-Z]/ -> char_rotate(char, shift, 65)
      char =~ ~r/[a-z]/ -> char_rotate(char, shift, 97)
      true -> char
    end
  end

  def char_rotate(char, shift, basecodepoint) do
    <<eacute :: utf8>> = char
    <<basecodepoint + Integer.mod(eacute - basecodepoint + shift, 26)>>
  end
end
