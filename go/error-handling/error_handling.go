package erratum

func Use(o ResourceOpener, input string) (useError error) {
again:
	res, err := o()
	if err != nil {
		if _, ok := err.(TransientError); ok {
			goto again
		}
		useError = err
		return
	}
	defer func() {
		res.Close()
	}()
	defer func() {
		if err := recover(); err != nil {
			switch recoverError := err.(type) {
			case FrobError:
				res.Defrob(recoverError.defrobTag)
				useError = recoverError.inner
			case error:
				useError = recoverError
			}
		}
	}()
	res.Frob(input)
	return
}
