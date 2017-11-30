defmodule SecretHandshake do
  use Bitwise

  @pair [
    {0b00001, "wink"},
    {0b00010, "double blink"},
    {0b00100, "close your eyes"},
    {0b01000, "jump"}
  ]

  @doc """
  Determine the actions of a secret handshake based on the binary
  representation of the given `code`.

  If the following bits are set, include the corresponding action in your list
  of commands, in order from lowest to highest.

  1 = wink
  10 = double blink
  100 = close your eyes
  1000 = jump

  10000 = Reverse the order of the operations in the secret handshake
  """
  @spec commands(code :: integer) :: list(String.t())
  def commands(code) do
    @pair
    |> Enum.map(fn({flag, word}) -> command_impl(code, flag, word) end)
    |> Enum.reject(&is_nil/1)
    |> SecretHandshake.command_impl2(code)
  end

  def command_impl(code, flag, word) do
    if ((code &&& flag) != 0), do: word, else: nil
  end

  def command_impl2(words, code) do
    if ((code &&& 0b10000) != 0), do: Enum.reverse(words), else: words
  end
end
