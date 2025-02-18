module SimpleFSM(input CLOCK_50, input [2:0] KEY, output [5:0] LEDR);

localparam [1:0] OFF = 2'b00, ON1 = 2'b01, ON2 = 2'b10, BOTH = 2'b11;
reg [1:0] state, next_state;

assign LEDR[1:0] = state;

reg [3:0] count;
assign LEDR[5:2] = count;

always @(negedge KEY[2]) begin
	count = count + 1;
	state <= next_state;  // State transition on clock edge
end

always @(negedge KEY[0]) begin
	case (state)
		OFF:  next_state <= BOTH;
		ON1:  next_state <= OFF;
		ON2:  next_state <= ON1;
		BOTH: next_state <= ON2;
		default: next_state <= OFF;
	endcase
end

always @(negedge KEY[1]) begin
	case (state)
		OFF:  next_state <= ON1;
		ON1:  next_state <= ON2;
		ON2:  next_state <= BOTH;
		BOTH: next_state <= OFF;
		default: next_state <= OFF;
	endcase
end

endmodule
