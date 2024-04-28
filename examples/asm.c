void main() {
	/* Variables & arithmetic */

	int x = 1;
	// LDA #$01
	// STA $0000

	int a = x + 2;
	// ADC #$02
	// STA $0001

	int s = a - 3;
	// SBC #$05
	// STA $0002

	int m = 3 * 3;
	// LDA #$03
	// 	ADC #$03
	// ADC #$03
	// STA $0003
	
	int d = 12 / 3;
}