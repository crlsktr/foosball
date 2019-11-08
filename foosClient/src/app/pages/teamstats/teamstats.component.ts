import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { FoosService } from 'src/services/foos.service';
import * as d3 from 'd3';

interface HistoryPoint {
	played: Date,
	ranking: number
}

@Component({
	selector: 'app-team-stats',
	templateUrl: './teamstats.component.html',
	styleUrls: ['./teamstats.component.scss']
})
export class TeamStatsComponent implements OnInit {


	constructor(private route: ActivatedRoute, private foosService: FoosService, private router: Router) { }

  public details: any;
	public teamId = 0;
	public stats : [];
	public statsKeys;
	public loadingMessage = 'Loading player stats ...';
	private margin : {left:number, right: number, top: number, bottom: number} = {left: 20,right: 20, top: 20, bottom: 20};
	private width : number = 300;
	private height: number = 300;

	ngOnInit() {
		if (!this.foosService.loggedIn) {
			this.router.navigateByUrl(`login`);
		}

		this.route.params.subscribe( params => {
      this.teamId = +params['teamId'];
      this.loadTeamDetail();
			this.loadStats();
		});
	}
  loadTeamDetail() {
    this.foosService.getTeamDetail(this.teamId)
    .then((detail)=> {
      this.details = detail;
    })
  }

	private loadStats() {
			this.foosService.getTeamGames(this.teamId)
			.then((stats) => {
				this.stats = stats;
				this.drawHistory(stats);
			})
			.catch((err) => {
				this.loadingMessage = `Couldn't find any stats for the team, ${err}`;
			});
	}

	drawHistory(hist: HistoryPoint[]) {
		let xScale = d3
			.scaleTime()
			.domain(d3.extent(hist, x => x.played))
			.range([this.margin.left, this.width -this.margin.right]);
		let yScale = d3
			.scaleLinear()
			.domain([d3.min(hist, x=>x.ranking), d3.max(hist, x=>x.ranking)])
			.nice()
			.range([this.height - this.margin.bottom, this.margin.top]);
		let xAxis = (g: d3.Selection<d3.BaseType, {}, HTMLElement, any>) =>
			g
				.attr('transform',`translate(0, ${this.height - this.margin.bottom})`)
				.call(
					d3
						.axisBottom(xScale)
						.ticks(this.width / 80)
						.tickFormat((value: Date) => value.toLocaleString())
						.tickSizeOuter(0)
				)
				.selectAll('text')
				.style('text-anchor','end')
				.attr('dx', '-.8em')
				.attr('dy', '0.15em')
				.attr('transform','rotate(-90');
		let yAxis = (g: d3.Selection<d3.BaseType, {}, HTMLElement, any>) =>
					g
					.attr('transform', `translate(${this.margin.left},0)`)
					.call(
						d3
							.axisLeft(yScale)
							.ticks(8)
							//.tickFormat(val => val.toString())
					)
					.call(g => g.select('.domain').remove());

    let svg = d3.select("#historyplot");
    svg.append("circle")
    .style("stroke", "gray")
    .style("fill", "white")
    .attr("r", 40)
    .attr("cx",50)
    .attr("cy",50);

    console.log("xkcd: selected", svg);
		svg.append('g').call(xAxis);
		svg.append('g').call(yAxis);

		let lineGenerator = d3.line<HistoryPoint>()
			.x(d => xScale(d.played))
			.y(d => yScale(d.ranking));

		svg
			.append('path')
			.datum(hist)
			.attr('stroke', '#f2a003')
			.attr('stroke-width','3px')
			.attr('d', lineGenerator(hist));

		svg.selectAll()
			.data(hist)
			.enter()
			.append('circle')
			.attr('r', '5px')
			.attr('cx', (d: HistoryPoint) => xScale(d.played))
			.attr('cy', (d: HistoryPoint) => yScale(d.ranking))
			.attr('fill', '#f2a003')
			.on('mouseover', d => {
				console.log("xkcd: testing mouse in", d);
			})
			.on('mouseout', d => {
				console.log('xkcd: testing mouse out', d);
			});

		// let yScale

	}
}
