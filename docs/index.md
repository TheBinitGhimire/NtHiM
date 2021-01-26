<h1 align="center" style="font-size:36px;font-weight:bold;"> NtHiM</h1>
<h4 align="center"> <strong>Powered by BINIT GHIMIRE (<a href='https://twitter.com/WHOISbinit' target="_blank">@WHOISbinit</a>)</strong></h4>
<p align="center"> <img src="https://raw.githubusercontent.com/TheBinitGhimire/NtHiM/main/src/screenshot.png"/></p>
<h3 align="center"><strong>N<strong>ow, <strong>t</strong>he <strong>H</strong>ost <strong>i</strong>s <strong>M</strong>ine!</h3>
<hr/>
<h2 align="center">Super Fast Sub-domain Takeover Detection!</h2>
<p align="center"> <a href="https://www.rust-lang.org/" target="_blank"><img src="https://ForTheBadge.com/images/badges/made-with-rust.svg"/></a></p>
<hr>
<h2 id="installation">Installation</h2>
<h3 id="method-1-using-pre-compiled-binaries">Method 1: Using Pre-compiled Binaries</h3>
<p>The pre-compiled binaries for different systems are available in the <a href="https://github.com/TheBinitGhimire/NtHiM/releases"><strong>Releases</strong></a> page. You can download the one suitable for your system, unzip the file and start using NtHiM.</p>
<h3 id="method-2-manual-build">Method 2: Manual Build</h3>
<p>You will need Cargo to perform the manual build for NtHiM.If you have Cargo installed, you can simply follow the steps below:</p>
<ol>
   <li>Clone this repository, <code>git clone https://github.com/TheBinitGhimire/NtHiM</code>;</li>
   <li>Go inside the folder, <code>cd NtHiM</code>;</li>
   <li>Use the <code>cargo build</code> command,</li>
   <li>Go inside the newly-created <strong>target</strong> folder, and open the <strong>debug</strong> folder inside it, <code>cd target/debug</code>;</li>
   <li>You will find <strong>NtHiM.exe</strong> (on Microsoft Windows) or <strong>NtHiM</strong> binary (on Linux).</li>
</ol>
<hr>
<h2 id="usage">Usage</h2>
<table>
   <thead>
      <tr>
         <th>Flag</th>
         <th>Description</th>
         <th>Example</th>
      </tr>
   </thead>
   <tbody>
      <tr>
         <td>-h</td>
         <td>Display help related to usage!</td>
         <td>NtHiM -h</td>
      </tr>
      <tr>
         <td>-t</td>
         <td>Scan a single target!</td>
         <td>NtHiM -t https://example.example.com</td>
      </tr>
      <tr>
         <td>-f</td>
         <td>Scan a list of targets from a file!</td>
         <td>NtHiM -f hostnames.txt</td>
      </tr>
      <tr>
         <td>-c</td>
         <td>Number of Concurrent Threads!</td>
         <td>NtHiM -c 100 -f hostnames.txt</td>
      </tr>
      <tr>
         <td>-V</td>
         <td>Display the version information!</td>
         <td>NtHiM -V</td>
      </tr>
   </tbody>
</table>
<hr>
<h3 id="use-case-1-single-target-">Use Case 1 (Single Target):</h3>
<pre><code class="lang-bash">NtHiM -t <span class="hljs-string">https:</span><span class="hljs-comment">//example.example.com</span>
</code></pre>
<h3 id="use-case-2-multiple-targets-">Use Case 2 (Multiple Targets):</h3>
<pre><code class="lang-bash">NtHiM <span class="hljs-_">-f</span> hostnames.txt
</code></pre>
<hr>
<h3 id="usage-demonstration-">Usage Demonstration:</h3>
<p><img src="https://raw.githubusercontent.com/TheBinitGhimire/NtHiM/main/src/demonstration.gif" alt="NtHiM Usage Demonstration"></p>
<hr>
<h2 id="examples">Examples</h2>
<h3 id="single-target">Single Target</h3>
<p><img src="https://raw.githubusercontent.com/TheBinitGhimire/NtHiM/main/src/example1.png" alt="Single Target"></p>
<h3 id="multiple-targets-using-concurrent-threads">Multiple Targets using Concurrent Threads</h3>
<p><img src="https://raw.githubusercontent.com/TheBinitGhimire/NtHiM/main/src/example2.png" alt="Multiple Targets using Concurrent Threads"></p>
<hr>
<h2 id="workflow">Workflow</h2>
<h3 id="platform-identification">Platform Identification</h3>
<p><strong>NtHiM</strong> uses the data provided in <strong><a href="https://github.com/EdOverflow/can-i-take-over-xyz">EdOverflow/can-i-take-over-xyz</a></strong> for the platform identification.</p>
<hr>
<h2 id="contributions-and-feature-requests">Contributions and Feature Requests</h2>
<p align="center"> <a href="https://github.com/TheBinitGhimire/NtHiM/pulls"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square"/></a></p>
<p>If you are interested in contributing in the development of <strong>NtHiM</strong>, you can feel free to create a <strong>Pull Request</strong> with modifications in the original code, or you shall open up a new <strong>issue</strong>, and I will try to include the feature as requested.</p>
<p>There is no restriction on anyone for contributing to the development of <strong>NtHiM</strong>. If you would like to contribute, you can feel free to do so.</p>