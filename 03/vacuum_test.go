package main

import "testing"

func TestTEST1(t *testing.T) {
	test1Result := drive("TEST1")
	if test1Result != 3 {
		t.Errorf("vacuum.go failed, expected %v, got %v", 3, test1Result)
	}
}

func TestTEST2(t *testing.T) {
	test2Result := drive("TEST2")
	if test2Result != 3 {
		t.Errorf("vacuum.go failed, expected %v, got %v", 3, test2Result)
	}
}

func TestTEST3(t *testing.T) {
	test3Result := drive("TEST3")
	if test3Result != 11 {
		t.Errorf("vacuum.go failed, expected %v, got %v", 11, test3Result)
	}
}
