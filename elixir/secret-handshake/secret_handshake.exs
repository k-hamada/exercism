defmodule SecretHandshake do
  use Bitwise

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
    secret_hand_shake(code, [])
  end

  def secret_hand_shake(code, words) when (code &&& 0b00001) != 0 do secret_hand_shake((code ^^^ 0b00001), words ++ ["wink"]           ) end
  def secret_hand_shake(code, words) when (code &&& 0b00010) != 0 do secret_hand_shake((code ^^^ 0b00010), words ++ ["double blink"]   ) end
  def secret_hand_shake(code, words) when (code &&& 0b00100) != 0 do secret_hand_shake((code ^^^ 0b00100), words ++ ["close your eyes"]) end
  def secret_hand_shake(code, words) when (code &&& 0b01000) != 0 do secret_hand_shake((code ^^^ 0b01000), words ++ ["jump"]           ) end
  def secret_hand_shake(code, words) when (code &&& 0b10000) != 0 do secret_hand_shake((code ^^^ 0b10000), Enum.reverse(words)         ) end
  def secret_hand_shake(_, words) do words end
end
