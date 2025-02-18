module ToBCD(
	input [19:0] number,
	output [3:0] bcd0, bcd1, bcd2, bcd3, bcd4, bcd5);
	
	integer i, k;
	reg [3:0] bcd_buf [5:0];

	assign bcd0 = bcd_buf[0]; 
	assign bcd1 = bcd_buf[1]; 
	assign bcd2 = bcd_buf[2]; 
	assign bcd3 = bcd_buf[3]; 
	assign bcd4 = bcd_buf[4]; 
	assign bcd5 = bcd_buf[5];

	always @ (*) begin
		bcd_buf[0] = 4'd0;
		bcd_buf[1] = 4'd0;
		bcd_buf[2] = 4'd0;
		bcd_buf[3] = 4'd0;
		bcd_buf[4] = 4'd0;
		bcd_buf[5] = 4'd0;
		
		//shift 20 times
		for (i=19; i>=0; i=i-1) begin
	
			//check all 6 BCD tetrads, if >=5 then add 3 
			for (k=5; k>=0; k=k-1) begin
				if(bcd_buf[k] >= 5) begin
					bcd_buf[k] = bcd_buf[k] + 4'd3;
				end
			end
			
			//for the first 5 tetrads
			for (k=5; k>=1; k=k-1) begin
				bcd_buf[k] = bcd_buf[k] << 1; 
				bcd_buf[k][0] = bcd_buf[k-1][3];
			end	
		
			bcd_buf[0] = bcd_buf[0] << 1;
			bcd_buf[0][0] = number[i];
		end
	end

endmodule