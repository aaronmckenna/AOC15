package main

import "testing"

func TestTEST1(t *testing.T) {
	test1Result := drive()
	if test1Result != 2 {
		t.Errorf("vacuum.go failed, expected %v, got %v", 2, test1Result)
	}
}

func TestTEST2(t *testing.T) {
	test2Result := drive()

}
