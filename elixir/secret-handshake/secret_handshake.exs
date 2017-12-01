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
    case hand_shake(code) do
      words when ((code &&& 0b10000) != 0) ->
        Enum.reverse(words)
      words ->
        words
    end
  end

  def hand_shake(code) when (code &&& 0b00001) != 0 do ["wink"             | hand_shake(code ^^^ 0b00001)] end
  def hand_shake(code) when (code &&& 0b00010) != 0 do ["double blink"     | hand_shake(code ^^^ 0b00010)] end
  def hand_shake(code) when (code &&& 0b00100) != 0 do ["close your eyes"  | hand_shake(code ^^^ 0b00100)] end
  def hand_shake(code) when (code &&& 0b01000) != 0 do ["jump"             | hand_shake(code ^^^ 0b01000)] end
  def hand_shake(_) do [] end
end
