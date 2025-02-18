module ReactionGame(
	input CLOCK_50,
	input [3:0] KEY,
	output [9:0] LEDR,
	output [6:0] HEX0, HEX1, HEX2, HEX3, HEX4, HEX5);

	wire clk_ms;
	ClockDivider #(.prescaler(49999)) (.clk_in(CLOCK_50), .clk_out(clk_ms));

	wire [19:0] count_ms;
	Counter #(.bits(20)) (.clk(clk_ms), .rst(~KEY[0]), .count(count_ms));
	
	wire [3:0] bcd0, bcd1, bcd2, bcd3, bcd4, bcd5;
	ToBCD(count_ms, bcd0, bcd1, bcd2, bcd3, bcd4, bcd5);
	
	SegmentDecoder hex0_decoder(bcd0, HEX0);
	SegmentDecoder hex1_decoder(bcd1, HEX1);
	SegmentDecoder hex2_decoder(bcd2, HEX2);
	SegmentDecoder hex3_decoder(bcd3, HEX3);
	SegmentDecoder hex4_decoder(bcd4, HEX4);
	SegmentDecoder hex5_decoder(bcd5, HEX5);
	
	
endmodule