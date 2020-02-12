import { NextPage } from "next";
import Head from "next/head";
import { useRouter } from "next/router";
import React from "react";
import YTPlayer from "yt-player";

import styles from "~/components/styles.module.scss";

const YtTimerPage: NextPage = () => {
  return (
    <section className={styles.timerPage}>
      <script
        dangerouslySetInnerHTML={{
          __html:
            `'use strict';
            
  window.YTAPILoaded =
  window.YTAPILoaded ||
  new Promise(resolve => {
    window.onYouTubeIframeAPIReady = () => {
      resolve(window.YT);
    };
  });

            YTAPILoaded.then(async () => {
    const projectName = /^[a-z0-9\\-]+\\.glitch\\.me$/.test(document.location.host) ? document.location.host.split('.')[0] : null;
      
    if (projectName) {
      const h1 = document.querySelector('h1');
      const link = document.createElement('a');
      link.href = ` +
            "`https://glitch.com/edit/#!/${projectName}`" +
            `;
      link.textContent = 'edit';
      h1.appendChild(document.createTextNode(' ('));
      h1.appendChild(link);
      h1.appendChild(document.createTextNode(')'));
    }
      
    const params = new URLSearchParams(location.search);
    const videoIdOrUrl = params.get('v') || params.get('video_id') || params.get('event_id');
    const videoId = videoIdOrUrl.replace(/^.*[\\?\\&]v=/, '').replace(/\\?.*$/, '').replace(/^.*\\//, '');
    params.set('v', videoId);

    const v = document.querySelector('[name=v]');
    history.replaceState(undefined, '', '?' + params.toString());

    if (!videoId) return;
    
    const YT = await window.YTAPILoaded;
    
    const startTime = document.querySelector('#startTime');
    const endTime = document.querySelector('#endTime');
    const elapsedTime = document.querySelector('#elapsedTime');
    const elapsedDetail = document.querySelector('#elapsedDetail');
    const startCap = document.querySelector('#startCap');
    const endCap = document.querySelector('#endCap');
    const gotoStart = document.querySelector('#gotoStart');
    const gotoSnd = document.querySelector('#gotoSnd');
    const resetStart = document.querySelector('#resetStart');
    const resetEnd = document.querySelector('#resetEnd');

    document.addEventListener('keypress', ({key}) => {
      switch (key) {
        case 'i':
          startCap.focus();
          startCap.click();
          break;
        case 'o':
          endCap.focus();
          endCap.click();
          break;
      }
    });

    const updateElapsed = () => {
      let end = Number(endTime.value);
      let start = Number(startTime.value);
      elapsedTime.value = (end - start).toFixed(3);
      let negative = start > end;
      const elapsed = Math.abs(elapsedTime.value);
      const elapsedText = [
        ...(negative? ['-'] : []),
        (elapsed / 60 / 60) | 0, 'h ',
        ((elapsed / 60) % 60) | 0, 'm ' ,
        (elapsed % 60).toFixed(3).replace('.', 's '), 'ms'
      ].join('').replace(/^(0h|0m)/, '');
      elapsedDetail.textContent = elapsedText;
      if (negative) {
        elapsedDetail.classList.add('negative');
      } else {
        elapsedDetail.classList.remove('negative');
      }
      v.value = ` +
            "`https://youtu.be/${videoId}?t=${Math.floor(start)}`" +
            `;
      params.set('t', start);
      params.set('u', end); 
      history.replaceState(undefined, '', '?' + params.toString());
    };
      
    if (params.has('t')) {
      startTime.value = Number(params.get('t')).toFixed(3);
    }
    if (params.has('u')) {
      endTime.value = Number(params.get('u')).toFixed(3);
    }

    startTime.onchange = endTime.onchange = updateElapsed;

    const placeholder = document.getElementById('player');
    const player = new YT.Player(placeholder, {
      videoId: videoId,
      playerVars: {
        rel: 0, // related videos overlay
        controls: 1,
        iv_load_policy: 3, // disabled annotations
        loop: 1,
        modestbranding: 1,
        autoplay: 1,
        mute: 1,
        start: params.has('t') ? Math.floor(params.get('t')) : undefined,
        end: params.has('u') ? Math.ceil(params.get('u')) : undefined,
      },
      events: {
        onStateChange({data: state}) {
          if (state === 0) {
            // loop?
            // const seconds = Number(startTime.value);
            // player.seekTo(seconds);
            // player.pauseVideo();
          }
        },
        onReady() {
          if (!params.has('t')) {
            startTime.value = (0).toFixed(3);
          }

          if (!params.has('u')) {
            endTime.value = player.getDuration().toFixed(3);
          }

          updateElapsed();
          
          resetStart.onclick = () => {
            startTime.value = (0).toFixed(3);
            updateElapsed();
          };
          
          resetEnd.onclick = () => {
            endTime.value = player.getDuration().toFixed(3);
            updateElapsed();
          };
    
          startCap.onclick = () => {
            startTime.value = player.getCurrentTime().toFixed(3);
            if (+endTime.value < +startTime.value) {
              endTime.value = startTime.value;
            }
            updateElapsed();
          };
    
          endCap.onclick = () => {
            endTime.value = player.getCurrentTime().toFixed(3);
            if (+endTime.value < +startTime.value) {
              startTime.value = endTime.value;
            }
            updateElapsed();
          };
    
          gotoStart.onclick = () => {
            const seconds = Number(startTime.value);
            player.seekTo(seconds);
          };
    
          gotoEnd.onclick = () => {
            const seconds = Number(endTime.value);
            player.seekTo(seconds);
          };
        }
      }
    });
      
    const getVideoData = () => player.getVideoData ? player.getVideoData() : undefined;
      
  });`,
        }}
      />

      <h1>YouTube Video Timer</h1>

      <form>
        <label>
          Video URL:{" "}
          <input name="v" defaultValue="https://youtu.be/3gJHBA1go5I" />
        </label>
        <button type="submit">⤦ Load Video</button>
      </form>

      <div id="player"></div>

      <p>
        <label className="ca13513">
          Start Time:{" "}
          <input
            id="startTime"
            type="number"
            step="0.001"
            placeholder="1.000"
          />
        </label>
        <button id="startCap">↳ Get From Video</button>
        <button id="gotoStart">⬑ Seek Video To</button>
        <button id="resetStart">⇤ Reset</button>
      </p>

      <p>
        <label className="ca13513">
          End Time:{" "}
          <input
            id="endTime"
            type="number"
            step="0.001"
            placeholder="120.000"
          />
        </label>
        <button id="endCap">↳ Get From Video</button>
        <button id="gotoEnd">⬑ Seek Video To</button>
        <button id="resetEnd">⇥ Reset</button>
      </p>

      <p>
        <label className="ca13513">
          Duration:{" "}
          <input
            id="elapsedTime"
            type="number"
            step="0.001"
            placeholder="119.00"
          />
        </label>
        <span id="elapsedDetail"></span>
      </p>

      <p>
        <u>Tips:</u>
        <ul>
          <li>
            After clicking/focusing on the YouTube video player you can use the
            comma <kbd>,</kbd> and period <kbd>.</kbd> keys to seek backwards
            and forwards by single frames.
            <br />
          </li>
          <li>
            After clicking/focusing outside the YouTube player, you can use the{" "}
            <kbd>i</kbd> and <kbd>o</kbd> keys to capture the start and end time
            and end times from the player.
          </li>
        </ul>
      </p>

      <style jsx>{`
        :global(body > main) {
          width: 640px;
        }

        kbd {
          border: 2px outset #ccc;
          padding: 0px 2px;
          background: #eee;
        }

        .ca13513 {
          display: inline-block;
          width: 13em;
          text-align: right;
        }

        input {
          width: 6em;
          font-family: monospace;
        }

        [name="v"] {
          width: 22em;
        }

        button {
          padding: 0 8px;
          vertical-align: top;
          height: 32px;
        }

        button:focus {
          box-shadow: 0 0 6px 1px black;
        }

        #player {
          margin-top: 1em;
        }

        button::first-letter {
          font-family: monospace;
          font-weight: bold;
          font-size: 1.5em;
        }

        .negative {
          color: red;
        }

        #elapsedDetail {
          font-family: monospace;
          user-select: all;
        }
      `}</style>
    </section>
  );
};

export default YtTimerPage;
